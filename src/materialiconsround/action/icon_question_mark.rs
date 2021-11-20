
pub struct IconQuestionMark {
  props: crate::Props,
}

impl yew::Component for IconQuestionMark {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M7.92,7.54C7.12,7.2,6.78,6.21,7.26,5.49C8.23,4.05,9.85,3,11.99,3c2.35,0,3.96,1.07,4.78,2.41c0.7,1.15,1.11,3.3,0.03,4.9 c-1.2,1.77-2.35,2.31-2.97,3.45c-0.15,0.27-0.24,0.49-0.3,0.94c-0.09,0.73-0.69,1.3-1.43,1.3c-0.87,0-1.58-0.75-1.48-1.62 c0.06-0.51,0.18-1.04,0.46-1.54c0.77-1.39,2.25-2.21,3.11-3.44c0.91-1.29,0.4-3.7-2.18-3.7c-1.17,0-1.93,0.61-2.4,1.34 C9.26,7.61,8.53,7.79,7.92,7.54z M14,20c0,1.1-0.9,2-2,2s-2-0.9-2-2c0-1.1,0.9-2,2-2S14,18.9,14,20z"/></g></svg>
            </svg>
        }
    }
}


