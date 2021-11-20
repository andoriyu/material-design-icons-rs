
pub struct IconBedtime {
  props: crate::Props,
}

impl yew::Component for IconBedtime {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g opacity=".3"><path d="M9.27,4.49C6.2,5.6,4,8.55,4,12c0,4.41,3.59,8,8,8c1.81,0,3.54-0.62,4.93-1.71C13.02,16.9,7.64,12.03,9.27,4.49z"/></g><path d="M12.34,2.02c-0.12,0-0.23-0.01-0.35-0.01C6.4,2.01,2,6.54,2,12c0,5.52,4.48,10,10,10c3.71,0,6.93-2.02,8.66-5.02 C13.15,16.73,8.57,8.55,12.34,2.02z M12,20c-4.41,0-8-3.59-8-8c0-3.45,2.2-6.4,5.27-7.51c-1.63,7.54,3.75,12.41,7.66,13.8 C15.54,19.38,13.81,20,12,20z"/></g></g></svg>
            </svg>
        }
    }
}


