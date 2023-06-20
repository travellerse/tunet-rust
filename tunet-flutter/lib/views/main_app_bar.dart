import 'package:binding/binding.dart';
import 'package:flutter/material.dart';
import '../runtime.dart';

class MainAppBar extends StatefulWidget implements PreferredSizeWidget {
  const MainAppBar({super.key});

  @override
  State<StatefulWidget> createState() => _MainAppBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _MainAppBarState extends State<MainAppBar> {
  late PropertyChangedCallbackWrap<ManagedRuntime> busyCallback;

  @override
  void initState() {
    super.initState();

    final runtime = BindingSource.of<ManagedRuntime>(context);
    busyCallback = PropertyChangedCallbackWrap<ManagedRuntime>(
      source: runtime,
      callback: (runtime) => setState(() {}),
    );

    final provider = BindingProvider.of(context);
    provider.add(ManagedRuntime.logBusyProperty, busyCallback);
    provider.add(ManagedRuntime.onlineBusyProperty, busyCallback);
    provider.add(ManagedRuntime.detailBusyProperty, busyCallback);
  }

  @override
  void dispose() {
    final provider = BindingProvider.of(context);
    provider.remove(ManagedRuntime.logBusyProperty, busyCallback);
    provider.remove(ManagedRuntime.onlineBusyProperty, busyCallback);
    provider.remove(ManagedRuntime.detailBusyProperty, busyCallback);

    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final runtime = BindingSource.of<ManagedRuntime>(context);
    final logBusy = runtime.logBusy;
    final onlineBusy = runtime.onlineBusy;
    final detailBusy = runtime.detailBusy;
    return AppBar(
      leading: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Image.asset("assets/logo.png"),
      ),
      title: const Text('清华校园网'),
      actions: [
        IconButton(
          onPressed: logBusy
              ? null
              : () {
                  runtime.queueLogin();
                },
          icon: const Icon(Icons.login_rounded),
        ),
        IconButton(
          onPressed: logBusy
              ? null
              : () {
                  runtime.queueLogout();
                },
          icon: const Icon(Icons.logout_rounded),
        ),
        IconButton(
          onPressed: (logBusy || onlineBusy || detailBusy)
              ? null
              : () {
                  runtime.queueFlux();
                  runtime.queueOnlines();
                  runtime.queueDetails();
                },
          icon: const Icon(Icons.refresh_rounded),
        ),
      ],
    );
  }
}
