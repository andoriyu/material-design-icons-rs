
pub struct IconTouchApp {
  props: crate::Props,
}

impl yew::Component for IconTouchApp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M8.79,9.24V5.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5v3.74c1.21-0.81,2-2.18,2-3.74c0-2.49-2.01-4.5-4.5-4.5 s-4.5,2.01-4.5,4.5C6.79,7.06,7.58,8.43,8.79,9.24z M14.29,11.71c-0.28-0.14-0.58-0.21-0.89-0.21h-0.61v-6 c0-0.83-0.67-1.5-1.5-1.5s-1.5,0.67-1.5,1.5v10.74l-3.44-0.72c-0.37-0.08-0.76,0.04-1.03,0.31c-0.43,0.44-0.43,1.14,0,1.58 l4.01,4.01C9.71,21.79,10.22,22,10.75,22h6.1c1,0,1.84-0.73,1.98-1.72l0.63-4.47c0.12-0.85-0.32-1.69-1.09-2.07L14.29,11.71z"/></g></g></svg>
            </svg>
        }
    }
}


