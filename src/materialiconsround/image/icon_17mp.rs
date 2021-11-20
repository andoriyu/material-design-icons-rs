
pub struct Icon17mp {
  props: crate::Props,
}

impl yew::Component for Icon17mp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M7.75,5.5H9c0.55,0,1,0.45,1,1 v4.25c0,0.41-0.34,0.75-0.75,0.75S8.5,11.16,8.5,10.75V7H7.75C7.34,7,7,6.66,7,6.25S7.34,5.5,7.75,5.5z M12.5,17.75 c0,0.41-0.34,0.75-0.75,0.75S11,18.16,11,17.75V14h-1v2.25C10,16.66,9.66,17,9.25,17S8.5,16.66,8.5,16.25V14h-1v3.75 c0,0.41-0.34,0.75-0.75,0.75S6,18.16,6,17.75V13.5c0-0.55,0.45-1,1-1h4.5c0.55,0,1,0.45,1,1V17.75z M12,6.25 c0-0.41,0.34-0.75,0.75-0.75h2.75c0.67,0,1.15,0.65,0.96,1.29l-1.28,4.12c-0.11,0.35-0.43,0.59-0.8,0.59 c-0.56,0-0.97-0.54-0.8-1.08L14.62,7h-1.87C12.34,7,12,6.66,12,6.25z M18,16c0,0.55-0.45,1-1,1h-2v0.75 c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V13.5c0-0.55,0.45-1,1-1H17c0.55,0,1,0.45,1,1V16z"/><rect height="1.5" width="1.5" x="15" y="14"/></g></g></svg>
            </svg>
        }
    }
}

