
pub struct IconRailwayAlert {
  props: crate::Props,
}

impl yew::Component for IconRailwayAlert {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g opacity=".3"><path d="M4,16.5C4,17.33,4.67,18,5.5,18h9c0.83,0,1.5-0.67,1.5-1.5V13H4V16.5z M10,14c0.83,0,1.5,0.67,1.5,1.5S10.83,17,10,17 s-1.5-0.67-1.5-1.5S9.17,14,10,14z"/></g><g opacity=".3"><path d="M4.43,6H11c0-0.33,0.03-0.66,0.08-0.98C7.71,4.92,5.33,5.23,4.43,6z"/></g><path d="M4,11V8h7.29C11.1,7.37,11,6.7,11,6H4.43c0.9-0.77,3.28-1.08,6.65-0.98c0.1-0.7,0.3-1.37,0.59-1.99C2.97,2.67,2,5.02,2,7 v9.5C2,18.43,3.57,20,5.5,20L4,21v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V13c-1.91,0-3.63-0.76-4.89-2H4z M16,16.5 c0,0.83-0.67,1.5-1.5,1.5h-9C4.67,18,4,17.33,4,16.5V13h12V16.5z"/><circle cx="10" cy="15.5" r="1.5"/><g><path d="M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z"/></g></g></g></svg>
            </svg>
        }
    }
}


