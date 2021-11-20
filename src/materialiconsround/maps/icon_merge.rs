
pub struct IconMerge {
  props: crate::Props,
}

impl yew::Component for IconMerge {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M8.71,7.71c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41 c-0.39,0.39-1.02,0.39-1.41,0L13,6.83v5.1c0,1.06,0.42,2.08,1.17,2.83l4.12,4.12c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0 L12,15.41l-4.88,4.88c-0.39,0.39-1.02,0.39-1.41,0c-0.39-0.39-0.39-1.02,0-1.41l4.12-4.12c0.75-0.75,1.17-1.77,1.17-2.83v-5.1 l-0.88,0.88C9.73,8.1,9.1,8.1,8.71,7.71z"/></g></svg>
            </svg>
        }
    }
}


