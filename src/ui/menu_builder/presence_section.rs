use super::register_menu_item;
use crate::config::PersistentState;
use crate::types::{DeviceId, DeviceType, PresenceSource};
use crate::ui::{DeviceAction, MenuIdMap};
use tray_icon::menu::{CheckMenuItem, MenuItem, PredefinedMenuItem, Submenu};

pub(super) fn append_presence_settings(
    parent: &Submenu,
    device_id: &DeviceId,
    name: &str,
    device_type: DeviceType,
    state: &PersistentState,
    map: &mut MenuIdMap,
) -> anyhow::Result<()> {
    let settings = state.device_settings(device_id);
    let source = settings.map_or(PresenceSource::Windows, |s| s.presence_source);
    let submenu = Submenu::new("可用性", true);
    for (label, choice) in [
        ("Windows 连接状态", PresenceSource::Windows),
        ("A50 X 电源事件（实验性）", PresenceSource::AstroA50X),
    ] {
        let item = CheckMenuItem::new(label, true, source == choice, None);
        register_menu_item(
            map,
            item.id().clone(),
            DeviceAction::SetPresenceSource(choice),
            device_id,
            name,
            device_type,
        );
        submenu.append(&item)?;
    }
    if source == PresenceSource::AstroA50X {
        submenu.append(&PredefinedMenuItem::separator())?;
        submenu.append(&MenuItem::new("启用后请把耳机断电再接通一次", false, None))?;
    }
    parent.append(&submenu)?;
    Ok(())
}
