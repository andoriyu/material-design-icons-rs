
pub struct IconLinkedCamera {
  props: crate::Props,
}

impl yew::Component for IconLinkedCamera {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 20H4V8h4.05l1.83-2H15V4H9L7.17 6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9h-2v11zM16 2v1.33c2.58 0 4.66 2.09 4.67 4.66H22C21.99 4.68 19.31 2 16 2zm0 2.67V6c1.11 0 1.99.89 2 1.99h1.33c-.01-1.84-1.49-3.32-3.33-3.32z"/><path d="M14.98 10.01c-.13-.09-.26-.18-.39-.26.14.08.27.17.39.26zM17 9c0-.37-.11-.71-.28-1.01-.18-.3-.43-.55-.73-.72C15.7 7.1 15.36 7 15 7V6H9.88L8.05 8H4v12h16V9h-3zm-5 10c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z" opacity=".3"/><path d="M12 9c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0 8c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3z"/></svg>
            </svg>
        }
    }
}


