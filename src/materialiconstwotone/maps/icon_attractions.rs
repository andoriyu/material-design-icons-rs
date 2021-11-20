
pub struct IconAttractions {
  props: crate::Props,
}

impl yew::Component for IconAttractions {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><circle cx="11.98" cy="12.02" opacity=".3" r="1.5"/><path d="M20.15,14.42c0.23-0.77,0.35-1.58,0.35-2.42s-0.12-1.65-0.35-2.42c0.78-0.6,1.02-1.7,0.51-2.58 c-0.51-0.88-1.58-1.23-2.49-0.85c-1.11-1.17-2.56-2.03-4.18-2.42C13.85,2.75,13.01,2,12,2s-1.85,0.75-1.98,1.73 C8.39,4.12,6.95,4.98,5.83,6.15C4.92,5.77,3.85,6.12,3.34,7C2.83,7.88,3.07,8.98,3.85,9.58C3.62,10.35,3.5,11.16,3.5,12 s0.12,1.65,0.35,2.42c-0.78,0.6-1.02,1.7-0.51,2.58c0.51,0.88,1.58,1.23,2.49,0.85c0.4,0.42,0.83,0.79,1.3,1.12L5.78,22h1.88 l0.98-2.19c0.44,0.19,0.9,0.34,1.38,0.46C10.15,21.25,10.99,22,12,22s1.85-0.75,1.98-1.73c0.46-0.11,0.91-0.26,1.34-0.44L16.3,22 h1.88l-1.34-3c0.48-0.34,0.93-0.72,1.34-1.15c0.91,0.38,1.99,0.03,2.49-0.85S20.93,15.02,20.15,14.42z M13.56,18.75 C13.19,18.29,12.63,18,12,18s-1.2,0.29-1.57,0.75c-0.4-0.09-0.79-0.21-1.16-0.37l1.43-3.19c0.4,0.16,0.84,0.25,1.3,0.25 c0.44,0,0.87-0.08,1.26-0.23l1.42,3.18C14.32,18.54,13.95,18.66,13.56,18.75z M10.48,12.02c0-0.83,0.67-1.5,1.5-1.5 c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5C11.15,13.52,10.48,12.85,10.48,12.02z M18.71,14.01c-0.61,0.07-1.18,0.41-1.52,0.99 c-0.32,0.56-0.34,1.2-0.12,1.75c-0.28,0.29-0.58,0.55-0.9,0.79l-1.5-3.35c0.49-0.59,0.78-1.34,0.78-2.16 c0-1.89-1.55-3.41-3.46-3.41c-1.91,0-3.46,1.53-3.46,3.41c0,0.8,0.28,1.54,0.75,2.13l-1.52,3.39c-0.31-0.23-0.6-0.48-0.87-0.76 C7.15,16.23,7.13,15.57,6.8,15c-0.34-0.59-0.93-0.94-1.56-0.99c-0.22-0.68-0.33-1.4-0.33-2.15c0-0.64,0.09-1.26,0.25-1.85 c0.66-0.03,1.3-0.38,1.65-1c0.37-0.63,0.35-1.38,0.01-1.98C7.74,6.05,8.93,5.34,10.27,5c0.34,0.59,0.99,1,1.73,1s1.39-0.4,1.73-1 c1.34,0.34,2.53,1.07,3.44,2.05C16.85,7.64,16.84,8.38,17.2,9c0.35,0.6,0.96,0.95,1.6,1c0.16,0.59,0.25,1.21,0.25,1.86 C19.05,12.61,18.93,13.33,18.71,14.01z"/></g></g></svg>
            </svg>
        }
    }
}


