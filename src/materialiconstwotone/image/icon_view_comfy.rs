
pub struct IconViewComfy {
  props: crate::Props,
}

impl yew::Component for IconViewComfy {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M9.25 11h2.25v2H9.25zm0 4h2.25v2H9.25zm0-8h2.25v2H9.25zm4.25 8h2.25v2H13.5zM5 15h2.25v2H5zm0-4h2.25v2H5zm0-4h2.25v2H5zm12.75 0H20v2h-2.25zm-4.25 4h2.25v2H13.5zm0-4h2.25v2H13.5zm4.25 8H20v2h-2.25zm0-4H20v2h-2.25z" opacity=".3"/><path d="M3 5v14h19V5H3zm4.25 12H5v-2h2.25v2zm0-4H5v-2h2.25v2zm0-4H5V7h2.25v2zm4.25 8H9.25v-2h2.25v2zm0-4H9.25v-2h2.25v2zm0-4H9.25V7h2.25v2zm4.25 8H13.5v-2h2.25v2zm0-4H13.5v-2h2.25v2zm0-4H13.5V7h2.25v2zM20 17h-2.25v-2H20v2zm0-4h-2.25v-2H20v2zm0-4h-2.25V7H20v2z"/></svg>
            </svg>
        }
    }
}

