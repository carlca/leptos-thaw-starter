use leptos::*;
use thaw::*;

fn main() {
  mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Space>
      <Button variant=ButtonVariant::Primary>"Primary"</Button>
      <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
      <Button variant=ButtonVariant::Text>"Text"</Button>
      <Button variant=ButtonVariant::Link>"Link"</Button>
    </Space>
  }
}
