
pub struct IconMediation {
  props: crate::Props,
}

impl yew::Component for IconMediation {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><g><path d="M18,13h-5.06c-0.34,3.1-2.26,5.72-4.94,7.05C7.96,21.69,6.64,23,5,23c-1.66,0-3-1.34-3-3s1.34-3,3-3 c0.95,0,1.78,0.45,2.33,1.14c1.9-1.03,3.26-2.91,3.58-5.14h-3.1C7.4,14.16,6.3,15,5,15c-1.66,0-3-1.34-3-3s1.34-3,3-3 c1.3,0,2.4,0.84,2.82,2h3.1C10.6,8.77,9.23,6.9,7.33,5.86C6.78,6.55,5.95,7,5,7C3.34,7,2,5.66,2,4s1.34-3,3-3 c1.64,0,2.96,1.31,2.99,2.95c2.68,1.33,4.6,3.95,4.94,7.05H18V8l4,4l-4,4V13z"/></g></g></svg>
            </svg>
        }
    }
}


