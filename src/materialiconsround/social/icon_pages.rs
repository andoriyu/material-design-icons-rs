
pub struct IconPages {
  props: crate::Props,
}

impl yew::Component for IconPages {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 5v6h5l-.6-2.38c-.18-.74.48-1.4 1.22-1.22L11 8V3H5c-1.1 0-2 .9-2 2zm5 8H3v6c0 1.1.9 2 2 2h6v-5l-2.38.6c-.73.18-1.4-.48-1.21-1.21L8 13zm7.38 3.6L13 16v5h6c1.1 0 2-.9 2-2v-6h-5l.6 2.38c.18.74-.48 1.4-1.22 1.22zM19 3h-6v5l2.38-.6c.73-.18 1.4.48 1.21 1.21L16 11h5V5c0-1.1-.9-2-2-2z"/></svg>
            </svg>
        }
    }
}


