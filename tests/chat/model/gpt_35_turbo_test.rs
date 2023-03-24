use chatgpt::chat::model::Gpt35TurboChat;

#[test]
fn run_test() {
    let mut gpt = Gpt35TurboChat::new(
        env!("OPENAI_API_KEY")
    );
    println!("{}", gpt.run("재밌는 게임 추천 좀 해줘").unwrap());
    println!("{}", gpt.run("그 중에 가장 재밌는 게임으로 추천해줘").unwrap());
}

#[test]
fn run_with_n_test() {
    let mut gpt = Gpt35TurboChat::new(
        env!("OPENAI_API_KEY")
    );

    gpt.run("재밌는 게임 추천 좀 해줘").unwrap();
    let result = gpt.run_with_n("그 중에 가장 재밌는 게임으로 추천해줘", 2).unwrap();
    print(&result);

    assert_eq!(2, result.len());
}

fn print(messages: &[String]) {
    for s in messages.iter() {
        println!("{}", s);
    }
}
