use std::time::Duration;
use tauri::AppHandle;
use windows::core::{Interface, Result, HSTRING};
use windows::Data::Xml::Dom::{XmlDocument, XmlElement};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use windows::UI::Notifications::{
    NotificationSetting, ToastNotification, ToastNotificationManager, ToastTemplateType,
};
use windows::UI::Notifications::{ToastNotificationMode, ToastNotifier};

pub fn show_notification(
    app: &AppHandle,
    title: &str,
    body: &str,
    icon_src: Option<String>,
    silent: bool,
) -> Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }

    let xml = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastImageAndText02)?;
    set_text(&xml, title, body)?;

    if let Some(icon_src) = icon_src {
        set_circle_circle(&xml, &icon_src)?;
    }

    if silent {
        set_silent(&xml)?;
    }

    let app_id = app.config().identifier.clone();
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;
    let toast = ToastNotification::CreateToastNotification(&xml)?;

    notifier.Show(&toast)?;

    hide_toast_after(
        notifier.clone(),
        toast.clone(),
        Duration::from_secs(8),
    );

    Ok(())
}

pub fn show_notification_sound_preview(app: &AppHandle) -> Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }

    let xml = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastText02)?;
    set_text(&xml, "", "The notification sound looks like this")?;

    let app_id = app.config().identifier.clone();
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;
    let toast = ToastNotification::CreateToastNotification(&xml)?;

    notifier.Show(&toast)?;

    hide_toast_after(
        notifier.clone(),
        toast.clone(),
        Duration::from_secs(1),
    );

    Ok(())
}

pub fn is_silent_mode(app: &AppHandle) -> Result<bool> {
    let do_not_disturb = matches!(
        ToastNotificationManager::GetDefault()?.NotificationMode(),
        Ok(mode) if mode != ToastNotificationMode::Unrestricted
    );

    if do_not_disturb {
        return Ok(true);
    }

    let app_id = app.config().identifier.clone();
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;
    Ok(!matches!(notifier.Setting()?, NotificationSetting::Enabled))
}

fn set_text(xml: &XmlDocument, title: &str, body: &str) -> Result<()> {
    let node_list = xml.GetElementsByTagName(&HSTRING::from("text"))?;
    node_list.Item(0)?.SetInnerText(&title.into())?;
    node_list.Item(1)?.SetInnerText(&body.into())?;
    Ok(())
}

fn set_circle_circle(xml: &XmlDocument, src: &str) -> Result<()> {
    let node_list = xml.GetElementsByTagName(&HSTRING::from("image"))?;

    let node = node_list.Item(0)?;

    let element = node.cast::<XmlElement>()?;
    element.SetAttribute(&HSTRING::from("src"), &src.into())?;
    element.SetAttribute(&HSTRING::from("hint-crop"), &HSTRING::from("circle"))?;
    element.SetAttribute(&HSTRING::from("alt"), &HSTRING::from("Profile Icon"))?;

    Ok(())
}

fn set_silent(xml: &XmlDocument) -> Result<()> {
    let toast_nodes = xml.GetElementsByTagName(&HSTRING::from("toast"))?;
    let toast = toast_nodes.Item(0)?;

    let audio = xml.CreateElement(&HSTRING::from("audio"))?;
    audio.SetAttribute(&HSTRING::from("silent"), &HSTRING::from("true"))?;

    toast.AppendChild(&audio)?;
    Ok(())
}

fn hide_toast_after(notifier: ToastNotifier, toast: ToastNotification, delay: Duration) {
    tokio::spawn(async move {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        }

        tokio::time::sleep(delay).await;

        let _ = notifier.Hide(&toast);
    });
}
