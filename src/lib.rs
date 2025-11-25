use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[cfg(feature = "pretty_nice_input")]
pub use input::{
    CloseMenuAction, MenuInputOf, MenuInputs, MenuStackInput, MenuWithInput, close_menu_on_action,
    show_menu_on_action,
};

#[cfg(feature = "pretty_nice_input")]
mod input;

#[derive(Default)]
pub struct PrettyNiceMenusPlugin;

impl Plugin for PrettyNiceMenusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuStack::default())
            .add_systems(
                PostUpdate,
                (remove_despawned_menus, activate_stack_current).chain(),
            )
            .add_observer(show_mouse)
            .add_observer(hide_mouse)
            .add_observer(show_menus)
            .add_observer(hide_menus)
            .add_observer(despawn_menus);

        #[cfg(feature = "pretty_nice_input")]
        app.add_plugins(input::PrettyNiceMenusInputPlugin);
    }
}

#[derive(Component)]
pub struct MenuWithMouse;

#[derive(Component)]
pub struct MenuWithoutMouse;

#[derive(Component)]
pub struct MenuHidesWhenClosed;

#[derive(Component)]
pub struct MenuDespawnsWhenClosed;

#[derive(Resource, Default, Debug, Reflect)]
pub struct MenuStack {
    stack: Vec<Entity>,
    current_top: Option<Entity>,
}

impl MenuStack {
    pub fn push(&mut self, menu: Entity) {
        self.stack.push(menu);
        debug!("Pushed menu {menu:?}, stack is now {self:?}");
    }

    pub fn remove(&mut self, menu: Entity) {
        self.stack.retain(|&entity| entity != menu);
        debug!("Removed menu {menu:?}, stack is now {self:?}");
    }

    pub fn contains(&self, menu: Entity) -> bool {
        self.stack.contains(&menu)
    }

    pub fn toggle(&mut self, menu: Entity) {
        if self.contains(menu) {
            self.remove(menu);
        } else {
            self.push(menu);
        }
    }
}

#[derive(EntityEvent)]
pub struct ActivateMenu {
    #[event_target]
    pub menu: Entity,
}

#[derive(EntityEvent)]
pub struct DeactivateMenu {
    #[event_target]
    pub menu: Entity,
}

/// This is the main sync point for changing the menu stack to activating/deactivating menus.
fn activate_stack_current(mut menu_stack: If<ResMut<MenuStack>>, mut commands: Commands) -> Result {
    if !menu_stack.is_changed() {
        return Ok(());
    }

    let new_top = menu_stack.stack.last().cloned();

    if let Some(current_top) = menu_stack.current_top
        && new_top != Some(current_top)
    {
        commands.trigger(DeactivateMenu { menu: current_top });
        menu_stack.current_top = None;
    }

    if menu_stack.current_top.is_none()
        && let Some(new_top) = new_top
    {
        menu_stack.current_top = Some(new_top);
        commands.trigger(ActivateMenu { menu: new_top });
    }

    Ok(())
}

fn show_mouse(
    activate: On<ActivateMenu>,
    menus: Query<(), With<MenuWithMouse>>,
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if menus.get(activate.menu).is_ok()
        && let Ok(mut cursor_options) = cursor_options.single_mut()
    {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}

fn hide_mouse(
    activate: On<ActivateMenu>,
    menus: Query<(), With<MenuWithoutMouse>>,
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if menus.get(activate.menu).is_ok()
        && let Ok(mut cursor_options) = cursor_options.single_mut()
    {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
}

pub fn show_menu_on_event<Ev: EntityEvent>(input: On<Ev>, mut menu_stack: ResMut<MenuStack>) {
    menu_stack.push(input.event_target());
}

pub fn close_menu_on_event<Ev: EntityEvent>(input: On<Ev>, mut menu_stack: ResMut<MenuStack>) {
    menu_stack.remove(input.event_target());
}

fn show_menus(
    activate: On<ActivateMenu>,
    mut menus: Query<&mut Visibility, With<MenuHidesWhenClosed>>,
) {
    if let Ok(mut visibility) = menus.get_mut(activate.menu) {
        *visibility = Visibility::Visible;
    }
}

fn hide_menus(
    deactivate: On<DeactivateMenu>,
    mut menus: Query<&mut Visibility, With<MenuHidesWhenClosed>>,
) {
    if let Ok(mut visibility) = menus.get_mut(deactivate.menu) {
        *visibility = Visibility::Hidden;
    }
}

fn despawn_menus(
    deactivate: On<DeactivateMenu>,
    mut menus: Query<Entity, With<MenuDespawnsWhenClosed>>,
    mut commands: Commands,
) {
    if let Ok(menu) = menus.get_mut(deactivate.menu) {
        commands.entity(menu).despawn();
    }
}

fn remove_despawned_menus(
    mut menu_stack: ResMut<MenuStack>,
    mut commands: Commands,
    entities: Query<()>,
) {
    for menu in menu_stack.stack.clone() {
        if entities.get(menu).is_err() {
            menu_stack.remove(menu);
            commands.trigger(DeactivateMenu { menu });

            if menu_stack.current_top == Some(menu) {
                menu_stack.current_top = None;
            }
        }
    }
}
