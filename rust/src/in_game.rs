mod game_map;
mod island;

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn rng_index_test() {
        // rng的随机范围是[start, end)
        let mut rng = rand::thread_rng();
        assert_eq!(0, rng.gen_range(0..1))
    }
}
