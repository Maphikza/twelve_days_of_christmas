fn main() {
    song_verses();
}

fn song_verses() {
    let verses = [
        "On the first day of Christmas, my true love sent to me\nA partridge in a pear tree",
        "On the second day of Christmas, my true love sent to me\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the third day of Christmas, my true love sent to me\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the fourth day of Christmas, my true love sent to me\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the fifth day of Christmas, my true love sent to me\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the sixth day of Christmas, my true love sent to me\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the seventh day of Christmas, my true love sent to me\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the eighth day of Christmas, my true love sent to me\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the ninth day of Christmas, my true love sent to me\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the tenth day of Christmas, my true love sent to me\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the eleventh day of Christmas, my true love sent to me\nEleven pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
        "On the twelfth day of Christmas, my true love sent to me\nTwelve drummers drumming\nEleven pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree",
    ];

    for index_one in 0..12 {
        println!("{}\n", verses[index_one]);
    }
}
