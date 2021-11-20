
pub struct IconBento {
  props: crate::Props,
}

impl yew::Component for IconBento {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g opacity=".3"><path d="M4,17h8V7H4V17z M8,10.5c0.83,0,1.5,0.67,1.5,1.5S8.83,13.5,8,13.5S6.5,12.83,6.5,12S7.17,10.5,8,10.5z M14,13h6v4h-6V13z M20,7v4h-6V7H20z"/></g><g><path d="M20,5H4C2.9,5,2,5.9,2,7v10c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M20,11h-6V7h6V11z M4,7h8v10H4V7z M14,17v-4h6v4H14z M9.5,12c0,0.83-0.67,1.5-1.5,1.5S6.5,12.83,6.5,12s0.67-1.5,1.5-1.5S9.5,11.17,9.5,12z"/></g></svg>
            </svg>
        }
    }
}


