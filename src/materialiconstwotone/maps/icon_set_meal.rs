
pub struct IconSetMeal {
  props: crate::Props,
}

impl yew::Component for IconSetMeal {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M21,13H3V4h18V13z M20,6c-1.68,0-3.04,0.98-3.21,2.23C16.15,7.5,14.06,5.5,10.25,5.5c-4.67,0-6.75,3-6.75,3 s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73C16.96,10.02,18.32,11,20,11V6z" opacity=".3"/><path d="M21.05,17.56L3.08,18.5L3,17l17.98-0.94L21.05,17.56z M21,19.48H3v1.5h18V19.48z M23,13V4c0-1.1-0.9-2-2-2H3 C1.9,2,1,2.9,1,4v9c0,1.1,0.9,2,2,2h18C22.1,15,23,14.1,23,13z M21,13H3V4h18V13z M20,6c-1.68,0-3.04,0.98-3.21,2.23 C16.15,7.5,14.06,5.5,10.25,5.5c-4.67,0-6.75,3-6.75,3s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73C16.96,10.02,18.32,11,20,11V6z"/></svg>
            </svg>
        }
    }
}


