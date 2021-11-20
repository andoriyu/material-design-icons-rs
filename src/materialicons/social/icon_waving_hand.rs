
pub struct IconWavingHand {
  props: crate::Props,
}

impl yew::Component for IconWavingHand {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M23,17c0,3.31-2.69,6-6,6v-1.5c2.48,0,4.5-2.02,4.5-4.5H23z M1,7c0-3.31,2.69-6,6-6v1.5C4.52,2.5,2.5,4.52,2.5,7H1z M8.01,4.32l-4.6,4.6c-3.22,3.22-3.22,8.45,0,11.67s8.45,3.22,11.67,0l7.07-7.07c0.49-0.49,0.49-1.28,0-1.77 c-0.49-0.49-1.28-0.49-1.77,0l-4.42,4.42l-0.71-0.71l6.54-6.54c0.49-0.49,0.49-1.28,0-1.77s-1.28-0.49-1.77,0l-5.83,5.83l-0.71-0.71 l6.89-6.89c0.49-0.49,0.49-1.28,0-1.77s-1.28-0.49-1.77,0l-6.89,6.89L11.02,9.8l5.48-5.48c0.49-0.49,0.49-1.28,0-1.77 s-1.28-0.49-1.77,0l-7.62,7.62c1.22,1.57,1.11,3.84-0.33,5.28l-0.71-0.71c1.17-1.17,1.17-3.07,0-4.24l-0.35-0.35l4.07-4.07 c0.49-0.49,0.49-1.28,0-1.77C9.29,3.83,8.5,3.83,8.01,4.32z"/></svg>
            </svg>
        }
    }
}


