
pub struct IconMoveDown {
  props: crate::Props,
}

impl yew::Component for IconMoveDown {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><rect height="3" opacity=".3" width="5" x="15" y="6"/><path d="M3,11c0,2.45,1.76,4.47,4.08,4.91l-1.49-1.49L7,13l4,4.01L7,21l-1.41-1.41l1.58-1.58l0-0.06C3.7,17.54,1,14.58,1,11 c0-3.87,3.13-7,7-7h3v2H8C5.24,6,3,8.24,3,11z"/><path d="M22,11V4h-9v7H22z M20,9h-5V6h5V9z"/><rect height="7" width="9" x="13" y="13"/></g></g></svg>
            </svg>
        }
    }
}


