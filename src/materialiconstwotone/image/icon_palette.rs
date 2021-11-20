
pub struct IconPalette {
  props: crate::Props,
}

impl yew::Component for IconPalette {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><g><path d="M12,4c-4.41,0-8,3.59-8,8s3.59,8,8,8c0.28,0,0.5-0.22,0.5-0.5 c0-0.16-0.08-0.28-0.14-0.35c-0.41-0.46-0.63-1.05-0.63-1.65c0-1.38,1.12-2.5,2.5-2.5H16c2.21,0,4-1.79,4-4 C20,7.14,16.41,4,12,4z M6.5,13C5.67,13,5,12.33,5,11.5S5.67,10,6.5,10S8,10.67,8,11.5S7.33,13,6.5,13z M9.5,9 C8.67,9,8,8.33,8,7.5S8.67,6,9.5,6S11,6.67,11,7.5S10.33,9,9.5,9z M14.5,9C13.67,9,13,8.33,13,7.5S13.67,6,14.5,6 S16,6.67,16,7.5S15.33,9,14.5,9z M19,11.5c0,0.83-0.67,1.5-1.5,1.5S16,12.33,16,11.5s0.67-1.5,1.5-1.5S19,10.67,19,11.5z" enable-background="new" opacity=".3"/><path d="M12,2C6.49,2,2,6.49,2,12s4.49,10,10,10c1.38,0,2.5-1.12,2.5-2.5c0-0.61-0.23-1.21-0.64-1.67 c-0.08-0.09-0.13-0.21-0.13-0.33c0-0.28,0.22-0.5,0.5-0.5H16c3.31,0,6-2.69,6-6C22,6.04,17.51,2,12,2z M16,15h-1.77 c-1.38,0-2.5,1.12-2.5,2.5c0,0.61,0.22,1.19,0.63,1.65c0.06,0.07,0.14,0.19,0.14,0.35c0,0.28-0.22,0.5-0.5,0.5 c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.14,8,7C20,13.21,18.21,15,16,15z"/><circle cx="6.5" cy="11.5" r="1.5"/><circle cx="9.5" cy="7.5" r="1.5"/><circle cx="14.5" cy="7.5" r="1.5"/><circle cx="17.5" cy="11.5" r="1.5"/></g></g></g></g></svg>
            </svg>
        }
    }
}


