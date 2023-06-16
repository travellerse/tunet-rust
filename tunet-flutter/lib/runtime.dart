import 'dart:async';
import 'package:flutter/services.dart';

import 'ffi.dart';
export 'ffi.dart';

class ManagedRuntime {
  final Runtime runtime;

  static const statusApi = MethodChannel('com.berrysoft.tunet_flutter/status');

  late StreamController<bool> logBusySink;
  late StreamController<NetFlux> netFluxSink;
  late StreamController<NetState> stateSink;
  late StreamController<String> statusSink;

  late Stream<bool> logBusyStream;
  late Stream<NetFlux> netFluxStream;
  late Stream<NetState> stateStream;
  late Stream<String> statusStream;

  ManagedRuntime({required this.runtime}) {
    logBusySink = StreamController();
    logBusySink.add(false);
    logBusyStream = logBusySink.stream.asBroadcastStream();

    netFluxSink = StreamController();
    netFluxStream = netFluxSink.stream.asBroadcastStream();

    stateSink = StreamController();
    stateStream = stateSink.stream.asBroadcastStream();

    statusSink = StreamController();
    statusStream = statusSink.stream.asBroadcastStream();
  }

  static Future<ManagedRuntime> newRuntime() async {
    final runtime = await Runtime.newRuntime(bridge: api);
    return ManagedRuntime(runtime: runtime);
  }

  Future<void> start() async {
    NetStatusSimp sendStatus = NetStatusSimp.Unknown;
    String? ssid;
    final String? status = await statusApi.invokeMethod("getStatus");
    switch (status) {
      case "wwan":
        sendStatus = NetStatusSimp.Wwan;
        break;
      case "wlan":
        sendStatus = NetStatusSimp.Wlan;
        ssid = await statusApi.invokeMethod("getSsid");
        break;
      case "lan":
        sendStatus = NetStatusSimp.Lan;
        break;
    }
    await runtime.queueStatus(t: sendStatus, ssid: ssid);
    await for (final msgw in runtime.start()) {
      final msg = msgw.field0;
      switch (msg) {
        case UpdateMsg.State:
          await runtime.queueFlux();
          stateSink.add((await runtime.state()).field0);
          break;
        case UpdateMsg.Status:
          await runtime.queueState();
          statusSink.add((await runtime.status()));
          break;
        case UpdateMsg.Flux:
          netFluxSink.add(await runtime.flux());
          break;
        case UpdateMsg.LogBusy:
          logBusySink.add(await runtime.logBusy());
          break;
        default:
          break;
      }
    }
  }

  Future<void> queueState({NetState? s}) =>
      runtime.queueState(s: s != null ? NetStateWrap(field0: s) : null);

  Future<void> queueLogin() => runtime.queueLogin();
  Future<void> queueLogout() => runtime.queueLogout();
  Future<void> queueFlux() => runtime.queueFlux();
}
