
pub struct IconExtension {
  props: crate::Props,
}

impl yew::Component for IconExtension {
 type Properties = crate::Props;
    type Message = ();

    fn create(props: Self::Properties, _: yew::prelude::ComponentLink<Self>) -> Self
    {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> yew::prelude::ShouldRender
    {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::prelude::ShouldRender
    {
        false
    }

    fn view(&self) -> yew::prelude::Html
    {
        yew::prelude::html! {
            <svg
                class=self.props.class.unwrap_or("")
                width=self.props.size.unwrap_or(24).to_string()
                height=self.props.size.unwrap_or(24).to_string()
                viewBox="0 0 24 24"
                fill=self.props.fill.unwrap_or("none")
                stroke=self.props.color.unwrap_or("currentColor")
                stroke-width=self.props.stroke_width.unwrap_or(2).to_string()
                stroke-linecap=self.props.stroke_linecap.unwrap_or("round")
                stroke-linejoin=self.props.stroke_linejoin.unwrap_or("round")
            >
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M20.36 11H19V5h-6V3.64c0-1.31-.94-2.5-2.24-2.63C9.26.86 8 2.03 8 3.5V5H2.01v5.8H3.4c1.31 0 2.5.88 2.75 2.16.33 1.72-.98 3.24-2.65 3.24H2V22h5.8v-1.4c0-1.31.88-2.5 2.16-2.75 1.72-.33 3.24.98 3.24 2.65V22H19v-6h1.5c1.47 0 2.64-1.26 2.49-2.76-.13-1.3-1.33-2.24-2.63-2.24z"/></svg>
            </svg>
        }
    }
}


