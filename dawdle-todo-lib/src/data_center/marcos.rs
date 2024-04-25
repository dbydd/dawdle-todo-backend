macro_rules! for_each_type_of_containers {
    (type $container_type:ident, $deserialize_code:block) => {{
        {
            type $container_type =
                crate::data_center::container::basic_priority_queue::BasicPriorityContainer;
            $deserialize_code
        }
        {
            type $container_type = crate::data_center::container::once::OnceContainer;
            $deserialize_code
        }
    }};
}
