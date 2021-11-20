
pub struct IconNordicWalking {
  props: crate::Props,
}

impl yew::Component for IconNordicWalking {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M18.25,23c-0.41,0-0.75-0.34-0.75-0.75V14H19v8.25C19,22.66,18.66,23,18.25,23z M4.93,23c0.35,0,0.66-0.24,0.73-0.59 L7.53,14H6l-1.8,8.09C4.1,22.56,4.45,23,4.93,23z M13.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S12.4,5.5,13.5,5.5z M14,23 c0.55,0,1-0.45,1-1v-5.64c0-0.55-0.22-1.07-0.62-1.45L12.9,13.5l0.6-3c1.07,1.24,2.62,2.13,4.36,2.41c0.6,0.1,1.14-0.38,1.14-0.99 c0-0.49-0.35-0.91-0.83-0.98c-1.53-0.24-2.79-1.14-3.47-2.33l-1-1.6c-0.56-0.89-1.68-1.25-2.66-0.84L7.22,7.78 C6.48,8.1,6,8.82,6,9.62V12c0,0.55,0.45,1,1,1s1-0.45,1-1V9.6l1.8-0.7L7.25,21.76C7.12,22.4,7.61,23,8.27,23 c0.49,0,0.91-0.34,1.02-0.81L10.9,15l2.1,2v5C13,22.55,13.45,23,14,23z"/></svg>
            </svg>
        }
    }
}


