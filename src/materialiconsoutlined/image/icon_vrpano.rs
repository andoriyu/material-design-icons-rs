
pub struct IconVrpano {
  props: crate::Props,
}

impl yew::Component for IconVrpano {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21.01,4C20.45,4,17.4,5.5,12,5.5c-5.31,0-8.49-1.49-9.01-1.49C2.46,4.01,2,4.45,2,5.02V19c0,0.57,0.46,1,0.99,1 c0.57,0,3.55-1.5,9.01-1.5c5.42,0,8.44,1.5,9.01,1.5c0.53,0,0.99-0.43,0.99-1V5C22,4.43,21.54,4,21.01,4z M20,17.63 c-2.01-0.59-4.62-1.13-8-1.13c-3.39,0-5.99,0.54-8,1.13V6.38C6.58,7.11,9.32,7.5,12,7.5c3.38,0,5.99-0.54,8-1.13V17.63z"/><path d="M9.17,10.99l-3.69,4.42C7.48,15.15,9.66,15,12,15c2.3,0,4.52,0.15,6.51,0.4L14,10l-2.83,3.39L9.17,10.99z"/></g></g></svg>
            </svg>
        }
    }
}


