use std::cell::RefCell;
use std::rc::Rc;

extern crate gio;
extern crate glib;
extern crate gtk;
// use gio::prelude::*;
use gtk::prelude::*;

use crate::app::{irc_bot, WidgetSetRef};

type SignalHandlerIdOptionRef = Rc<RefCell<Option<glib::SignalHandlerId>>>;

#[derive(Default)]
pub struct SignalHandlerIdSet {
    pub hostname_entry_changed: SignalHandlerIdOptionRef,
    pub port_spin_button_value_changed: SignalHandlerIdOptionRef,
    pub nick_entry_changed: SignalHandlerIdOptionRef,
    pub channel_entry_changed: SignalHandlerIdOptionRef,
    pub connect_disconnect_button_clicked: SignalHandlerIdOptionRef,
}

/// ホスト名欄が変更されたときの処理を登録する。
pub fn connect_hostname_entry_changed(
    widgets: &WidgetSetRef,
    irc_bot_config: &irc_bot::ConfigRef,
) -> glib::SignalHandlerId {
    let w = widgets.borrow();
    let c = irc_bot_config.clone();
    w.hostname_entry.connect_changed(move |e| {
        let mut c = c.borrow_mut();
        c.hostname = e.get_text().unwrap().to_string();
    })
}

/// ポートの値が変更されたときの処理を登録する。
pub fn connect_port_spin_button_value_changed(
    widgets: &WidgetSetRef,
    irc_bot_config: &irc_bot::ConfigRef,
) -> glib::SignalHandlerId {
    let w = widgets.borrow();
    let c = irc_bot_config.clone();
    w.port_spin_button.connect_value_changed(move |b| {
        let mut c = c.borrow_mut();
        c.port = b.get_value() as u16;
    })
}

/// ニックネーム欄が変更されたときの処理を登録する。
pub fn connect_nick_entry_changed(
    widgets: &WidgetSetRef,
    irc_bot_config: &irc_bot::ConfigRef,
) -> glib::SignalHandlerId {
    let w = widgets.borrow();
    let c = irc_bot_config.clone();
    w.nick_entry.connect_changed(move |e| {
        let mut c = c.borrow_mut();
        c.nick = e.get_text().unwrap().to_string();
    })
}

/// チャンネル欄が変更されたときの処理を登録する。
pub fn connect_channel_entry_changed(
    widgets: &WidgetSetRef,
    irc_bot_config: &irc_bot::ConfigRef,
) -> glib::SignalHandlerId {
    let w = widgets.borrow();
    let c = irc_bot_config.clone();
    w.channel_entry.connect_changed(move |e| {
        let mut c = c.borrow_mut();
        c.channel = e.get_text().unwrap().to_string();
    })
}

/// 接続・切断ボタンが押されたときの処理を登録する。
pub fn connect_connect_disconnect_button_clicked(
    widgets: &WidgetSetRef,
    irc_bot_config: &irc_bot::ConfigRef,
) -> glib::SignalHandlerId {
    let w = widgets.borrow();
    let c = irc_bot_config.clone();
    w.connect_disconnect_button.connect_clicked(move |_| {
        println!("irc_bot_config: {:?}", c.borrow());
    })
}
