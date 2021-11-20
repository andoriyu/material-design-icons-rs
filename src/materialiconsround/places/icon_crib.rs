
pub struct IconCrib {
  props: crate::Props,
}

impl yew::Component for IconCrib {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M18.32,18.32c-0.36-0.36-0.92-0.4-1.31-0.08c-0.32,0.25-0.65,0.48-1,0.69V16h2c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-6V6 c0-1.1-0.9-2-2-2H8C5.79,4,4,5.79,4,8v6c0,1.1,0.9,2,2,2h2v2.93c-0.35-0.2-0.69-0.43-1-0.69c-0.39-0.32-0.96-0.27-1.31,0.08 c-0.42,0.42-0.39,1.12,0.08,1.5C7.47,21.18,9.64,22,12,22c2.36,0,4.53-0.82,6.24-2.18C18.71,19.44,18.74,18.74,18.32,18.32z M14,19.75C13.36,19.91,12.69,20,12,20c-0.69,0-1.36-0.09-2-0.25V16h4V19.75z"/></svg>
            </svg>
        }
    }
}


