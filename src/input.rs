use bevy::prelude::*;
use bevy_pretty_nice_input::{Action, InputDisabled, JustPressed};

use crate::{MenuStack, remove_despawned_menus};

#[derive(Default)]
pub struct PrettyNiceMenusInputPlugin;

impl Plugin for PrettyNiceMenusInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuStackInput::default())
            .add_systems(
                PostUpdate,
                activate_stack_current_input.after(remove_despawned_menus),
            )
            .add_observer(disable_input_managers_on_add_menu_with_input)
            .add_observer(disable_input_managers_on_add_menu_input_of)
            .add_observer(close_menu_on_action::<CloseMenuAction>);
    }
}

#[derive(Resource, Default, Debug, Reflect)]
pub struct MenuStackInput {
    current_input: Option<Entity>,
}

#[derive(Component)]
pub struct MenuWithInput;

#[derive(Component)]
#[relationship_target(relationship = MenuInputOf)]
pub struct MenuInputs(#[relationship] Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = MenuInputs)]
pub struct MenuInputOf(#[relationship] pub Entity);

#[derive(Action)]
pub struct CloseMenuAction;

pub fn show_menu_on_action<Action: bevy_pretty_nice_input::Action, Menu: Component>(
    _: On<JustPressed<Action>>,
    mut menus: Query<Entity, With<Menu>>,
    mut menu_stack: ResMut<MenuStack>,
) -> Result {
    let menu = menus.single_mut()?;
    menu_stack.push(menu);
    Ok(())
}

pub fn close_menu_on_action<Action: bevy_pretty_nice_input::Action>(
    pressed: On<JustPressed<Action>>,
    mut menu_stack: ResMut<MenuStack>,
) {
    menu_stack.remove(pressed.input);
}

/// This is the main sync point for changing the menu stack to activating/deactivating menus.
fn activate_stack_current_input(
    menu_stack: If<Res<MenuStack>>,
    mut menu_stack_input: If<ResMut<MenuStackInput>>,
    menus_with_inputs: Query<(), With<MenuWithInput>>,
    menu_inputs: Query<&MenuInputs>,
    mut commands: Commands,
) -> Result {
    if !menu_stack.is_changed() {
        return Ok(());
    }

    let new_top_input = menu_stack
        .stack
        .iter()
        .rev()
        .find(|menu| menus_with_inputs.get(**menu).is_ok())
        .cloned();

    if let Some(current_input) = menu_stack_input.current_input
        && new_top_input != Some(current_input)
    {
        commands.entity(current_input).insert(InputDisabled);

        for input in menu_inputs.iter_descendants(current_input) {
            commands.entity(input).insert(InputDisabled);
        }

        menu_stack_input.current_input = None;
    }

    if menu_stack_input.current_input.is_none()
        && let Some(new_top_input) = new_top_input
    {
        commands.entity(new_top_input).insert(InputDisabled);

        for input in menu_inputs.iter_descendants(new_top_input) {
            commands.entity(input).insert(InputDisabled);
        }

        menu_stack_input.current_input = Some(new_top_input);
    }

    Ok(())
}

fn disable_input_managers_on_add_menu_with_input(
    add: On<Add, MenuWithInput>,
    mut commands: Commands,
) {
    commands.entity(add.entity).insert(InputDisabled);
}

fn disable_input_managers_on_add_menu_input_of(add: On<Add, MenuInputOf>, mut commands: Commands) {
    commands.entity(add.entity).insert(InputDisabled);
}
