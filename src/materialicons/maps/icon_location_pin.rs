
pub struct IconLocationPin {
  props: crate::Props,
}

impl yew::Component for IconLocationPin {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12,2L12,2C8.13,2,5,5.13,5,9c0,1.74,0.5,3.37,1.41,4.84c0.95,1.54,2.2,2.86,3.16,4.4c0.47,0.75,0.81,1.45,1.17,2.26 C11,21.05,11.21,22,12,22h0c0.79,0,1-0.95,1.25-1.5c0.37-0.81,0.7-1.51,1.17-2.26c0.96-1.53,2.21-2.85,3.16-4.4 C18.5,12.37,19,10.74,19,9C19,5.13,15.87,2,12,2z M12,11.75c-1.38,0-2.5-1.12-2.5-2.5s1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 S13.38,11.75,12,11.75z"/></svg>
            </svg>
        }
    }
}


