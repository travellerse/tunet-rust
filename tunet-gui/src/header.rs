use crate::*;

pub enum HeaderMsg {
    Info,
    Detail,
    Settings,
    About,
}

pub struct HeaderModel {}

impl Model for HeaderModel {
    type Msg = HeaderMsg;
    type Widgets = HeaderWidgets;
    type Components = ();
}

impl ComponentUpdate<MainModel> for HeaderModel {
    fn init_model(_parent_model: &MainModel) -> Self {
        HeaderModel {}
    }

    fn update(
        &mut self,
        msg: HeaderMsg,
        _components: &(),
        _sender: Sender<HeaderMsg>,
        parent_sender: Sender<MainMsg>,
    ) {
        let mode = match msg {
            HeaderMsg::Info => MainMode::Info,
            HeaderMsg::Detail => MainMode::Detail,
            HeaderMsg::Settings => MainMode::Settings,
            HeaderMsg::About => MainMode::About,
        };
        send!(parent_sender, MainMsg::Mode(mode));
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<HeaderModel, MainModel> for HeaderWidgets {
    view! {
        gtk::HeaderBar {
            set_title_widget = Some(&gtk::Box) {
                append = &gtk::Label {
                    add_css_class: "title",
                    set_label: "清华校园网",
                    set_margin_end: 5,
                },

                append = &gtk::Box {
                    add_css_class: "linked",

                    append: group = &gtk::ToggleButton {
                        set_label: "主页",
                        set_active: true,
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Info);
                            }
                        },
                    },
                    append = &gtk::ToggleButton {
                        set_label: "明细",
                        set_group: Some(&group),
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Detail);
                            }
                        },
                    },
                    append = &gtk::ToggleButton {
                        set_label: "设置",
                        set_group: Some(&group),
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::Settings);
                            }
                        },
                    },
                    append = &gtk::ToggleButton {
                        set_label: "关于",
                        set_group: Some(&group),
                        connect_toggled(sender) => move |btn| {
                            if btn.is_active() {
                                send!(sender, HeaderMsg::About);
                            }
                        },
                    },
                },
            }
        }
    }
}
