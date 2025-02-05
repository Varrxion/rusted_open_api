use example_program::example_app_controller::ExampleAppController;

mod example_program;

fn main() {
    let mut example_app_controller = ExampleAppController::new();
    example_app_controller.init();
}
