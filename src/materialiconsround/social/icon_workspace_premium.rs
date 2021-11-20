
pub struct IconWorkspacePremium {
  props: crate::Props,
}

impl yew::Component for IconWorkspacePremium {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M10.92,12.75L12,11.93l1.07,0.81c0.39,0.29,0.92-0.08,0.78-0.55l-0.42-1.36l1.2-0.95C15,9.6,14.79,9,14.31,9h-1.4 l-0.43-1.34c-0.15-0.46-0.8-0.46-0.95,0L11.09,9H9.68C9.21,9,9,9.6,9.37,9.89l1.19,0.95l-0.42,1.36 C10,12.67,10.53,13.04,10.92,12.75z M6,21.61c0,0.68,0.67,1.16,1.32,0.95L12,21l4.68,1.56C17.33,22.78,18,22.3,18,21.61v-6.33 c1.24-1.41,2-3.25,2-5.28c0-4.42-3.58-8-8-8s-8,3.58-8,8c0,2.03,0.76,3.87,2,5.28V21.61z M12,4c3.31,0,6,2.69,6,6s-2.69,6-6,6 s-6-2.69-6-6S8.69,4,12,4z"/></svg>
            </svg>
        }
    }
}


