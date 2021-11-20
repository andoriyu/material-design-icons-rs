
pub struct IconPivotTableChart {
  props: crate::Props,
}

impl yew::Component for IconPivotTableChart {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M21,5c0-1.1-0.9-2-2-2h-9v5h11V5z"/><path d="M3,19c0,1.1,0.9,2,2,2h3V10H3V19z"/><path d="M3,5v3h5V3H5C3.9,3,3,3.9,3,5z"/><path d="M17.65,9.35l-2.79,2.79C14.54,12.46,14.76,13,15.21,13H17v2c0,1.1-0.9,2-2,2h-2v-1.79c0-0.45-0.54-0.67-0.85-0.35 l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79c0.31,0.31,0.85,0.09,0.85-0.35V19h2c2.21,0,4-1.79,4-4v-2h1.79 c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.79C18.16,9.16,17.84,9.16,17.65,9.35z"/></g></g></svg>
            </svg>
        }
    }
}


