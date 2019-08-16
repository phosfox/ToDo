use todo_item;

#[cfg(test)]
mod tests {

    #[test]
    fn test_creation() {
        let item = TodoItem::new("test", false);
        assert_eq!(item.name, "test");
        assert_eq!(item.done = false);
    }
}
