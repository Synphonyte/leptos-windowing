#[derive(Copy, Clone, Debug)]
pub struct Book {
    pub id: u32,
    pub title: &'static str,
    pub author: &'static str,
}

pub const BOOKS: [Book; 100] = [
    Book {
        id: 1,
        title: "The Great Gatsby",
        author: "F. Scott Fitzgerald",
    },
    Book {
        id: 2,
        title: "The Grapes of Wrath",
        author: "John Steinbeck",
    },
    Book {
        id: 3,
        title: "Nineteen Eighty-Four",
        author: "George Orwell",
    },
    Book {
        id: 4,
        title: "Ulysses",
        author: "James Joyce",
    },
    Book {
        id: 5,
        title: "To Kill a Mockingbird",
        author: "Harper Lee",
    },
    Book {
        id: 6,
        title: "Pride and Prejudice",
        author: "Jane Austen",
    },
    Book {
        id: 7,
        title: "The Catcher in the Rye",
        author: "J.D. Salinger",
    },
    Book {
        id: 8,
        title: "One Hundred Years of Solitude",
        author: "Gabriel García Márquez",
    },
    Book {
        id: 9,
        title: "The Lord of the Rings",
        author: "J.R.R. Tolkien",
    },
    Book {
        id: 10,
        title: "Brave New World",
        author: "Aldous Huxley",
    },
    Book {
        id: 11,
        title: "Animal Farm",
        author: "George Orwell",
    },
    Book {
        id: 12,
        title: "The Chronicles of Narnia",
        author: "C.S. Lewis",
    },
    Book {
        id: 13,
        title: "Fahrenheit 451",
        author: "Ray Bradbury",
    },
    Book {
        id: 14,
        title: "Jane Eyre",
        author: "Charlotte Brontë",
    },
    Book {
        id: 15,
        title: "Wuthering Heights",
        author: "Emily Brontë",
    },
    Book {
        id: 16,
        title: "The Picture of Dorian Gray",
        author: "Oscar Wilde",
    },
    Book {
        id: 17,
        title: "Moby Dick",
        author: "Herman Melville",
    },
    Book {
        id: 18,
        title: "War and Peace",
        author: "Leo Tolstoy",
    },
    Book {
        id: 19,
        title: "Anna Karenina",
        author: "Leo Tolstoy",
    },
    Book {
        id: 20,
        title: "Crime and Punishment",
        author: "Fyodor Dostoevsky",
    },
    Book {
        id: 21,
        title: "The Brothers Karamazov",
        author: "Fyodor Dostoevsky",
    },
    Book {
        id: 22,
        title: "The Odyssey",
        author: "Homer",
    },
    Book {
        id: 23,
        title: "The Iliad",
        author: "Homer",
    },
    Book {
        id: 24,
        title: "Don Quixote",
        author: "Miguel de Cervantes",
    },
    Book {
        id: 25,
        title: "The Divine Comedy",
        author: "Dante Alighieri",
    },
    Book {
        id: 26,
        title: "Hamlet",
        author: "William Shakespeare",
    },
    Book {
        id: 27,
        title: "Romeo and Juliet",
        author: "William Shakespeare",
    },
    Book {
        id: 28,
        title: "Macbeth",
        author: "William Shakespeare",
    },
    Book {
        id: 29,
        title: "A Midsummer Night's Dream",
        author: "William Shakespeare",
    },
    Book {
        id: 30,
        title: "The Tempest",
        author: "William Shakespeare",
    },
    Book {
        id: 31,
        title: "Great Expectations",
        author: "Charles Dickens",
    },
    Book {
        id: 32,
        title: "A Tale of Two Cities",
        author: "Charles Dickens",
    },
    Book {
        id: 33,
        title: "Oliver Twist",
        author: "Charles Dickens",
    },
    Book {
        id: 34,
        title: "David Copperfield",
        author: "Charles Dickens",
    },
    Book {
        id: 35,
        title: "The Adventures of Huckleberry Finn",
        author: "Mark Twain",
    },
    Book {
        id: 36,
        title: "The Adventures of Tom Sawyer",
        author: "Mark Twain",
    },
    Book {
        id: 37,
        title: "Little Women",
        author: "Louisa May Alcott",
    },
    Book {
        id: 38,
        title: "The Secret Garden",
        author: "Frances Hodgson Burnett",
    },
    Book {
        id: 39,
        title: "Alice's Adventures in Wonderland",
        author: "Lewis Carroll",
    },
    Book {
        id: 40,
        title: "Through the Looking-Glass",
        author: "Lewis Carroll",
    },
    Book {
        id: 41,
        title: "The Hobbit",
        author: "J.R.R. Tolkien",
    },
    Book {
        id: 42,
        title: "Dune",
        author: "Frank Herbert",
    },
    Book {
        id: 43,
        title: "Foundation",
        author: "Isaac Asimov",
    },
    Book {
        id: 44,
        title: "The Hitchhiker's Guide to the Galaxy",
        author: "Douglas Adams",
    },
    Book {
        id: 45,
        title: "Ender's Game",
        author: "Orson Scott Card",
    },
    Book {
        id: 46,
        title: "The Time Machine",
        author: "H.G. Wells",
    },
    Book {
        id: 47,
        title: "The War of the Worlds",
        author: "H.G. Wells",
    },
    Book {
        id: 48,
        title: "Twenty Thousand Leagues Under the Sea",
        author: "Jules Verne",
    },
    Book {
        id: 49,
        title: "Around the World in Eighty Days",
        author: "Jules Verne",
    },
    Book {
        id: 50,
        title: "Journey to the Center of the Earth",
        author: "Jules Verne",
    },
    Book {
        id: 51,
        title: "Frankenstein",
        author: "Mary Shelley",
    },
    Book {
        id: 52,
        title: "Dracula",
        author: "Bram Stoker",
    },
    Book {
        id: 53,
        title: "The Strange Case of Dr. Jekyll and Mr. Hyde",
        author: "Robert Louis Stevenson",
    },
    Book {
        id: 54,
        title: "Treasure Island",
        author: "Robert Louis Stevenson",
    },
    Book {
        id: 55,
        title: "Robinson Crusoe",
        author: "Daniel Defoe",
    },
    Book {
        id: 56,
        title: "Gulliver's Travels",
        author: "Jonathan Swift",
    },
    Book {
        id: 57,
        title: "The Canterbury Tales",
        author: "Geoffrey Chaucer",
    },
    Book {
        id: 58,
        title: "Paradise Lost",
        author: "John Milton",
    },
    Book {
        id: 59,
        title: "The Scarlet Letter",
        author: "Nathaniel Hawthorne",
    },
    Book {
        id: 60,
        title: "The House of Seven Gables",
        author: "Nathaniel Hawthorne",
    },
    Book {
        id: 61,
        title: "Walden",
        author: "Henry David Thoreau",
    },
    Book {
        id: 62,
        title: "On the Road",
        author: "Jack Kerouac",
    },
    Book {
        id: 63,
        title: "The Sun Also Rises",
        author: "Ernest Hemingway",
    },
    Book {
        id: 64,
        title: "For Whom the Bell Tolls",
        author: "Ernest Hemingway",
    },
    Book {
        id: 65,
        title: "The Old Man and the Sea",
        author: "Ernest Hemingway",
    },
    Book {
        id: 66,
        title: "A Farewell to Arms",
        author: "Ernest Hemingway",
    },
    Book {
        id: 67,
        title: "The Sound and the Fury",
        author: "William Faulkner",
    },
    Book {
        id: 68,
        title: "As I Lay Dying",
        author: "William Faulkner",
    },
    Book {
        id: 69,
        title: "Light in August",
        author: "William Faulkner",
    },
    Book {
        id: 70,
        title: "Absalom, Absalom!",
        author: "William Faulkner",
    },
    Book {
        id: 71,
        title: "Invisible Man",
        author: "Ralph Ellison",
    },
    Book {
        id: 72,
        title: "Go Tell It on the Mountain",
        author: "James Baldwin",
    },
    Book {
        id: 73,
        title: "The Color Purple",
        author: "Alice Walker",
    },
    Book {
        id: 74,
        title: "Beloved",
        author: "Toni Morrison",
    },
    Book {
        id: 75,
        title: "Song of Solomon",
        author: "Toni Morrison",
    },
    Book {
        id: 76,
        title: "The Bluest Eye",
        author: "Toni Morrison",
    },
    Book {
        id: 77,
        title: "Their Eyes Were Watching God",
        author: "Zora Neale Hurston",
    },
    Book {
        id: 78,
        title: "Native Son",
        author: "Richard Wright",
    },
    Book {
        id: 79,
        title: "Black Boy",
        author: "Richard Wright",
    },
    Book {
        id: 80,
        title: "The Bell Jar",
        author: "Sylvia Plath",
    },
    Book {
        id: 81,
        title: "One Flew Over the Cuckoo's Nest",
        author: "Ken Kesey",
    },
    Book {
        id: 82,
        title: "Slaughterhouse-Five",
        author: "Kurt Vonnegut",
    },
    Book {
        id: 83,
        title: "Cat's Cradle",
        author: "Kurt Vonnegut",
    },
    Book {
        id: 84,
        title: "Breakfast of Champions",
        author: "Kurt Vonnegut",
    },
    Book {
        id: 85,
        title: "Catch-22",
        author: "Joseph Heller",
    },
    Book {
        id: 86,
        title: "Something Happened",
        author: "Joseph Heller",
    },
    Book {
        id: 87,
        title: "The Metamorphosis",
        author: "Franz Kafka",
    },
    Book {
        id: 88,
        title: "The Trial",
        author: "Franz Kafka",
    },
    Book {
        id: 89,
        title: "The Castle",
        author: "Franz Kafka",
    },
    Book {
        id: 90,
        title: "The Stranger",
        author: "Albert Camus",
    },
    Book {
        id: 91,
        title: "The Plague",
        author: "Albert Camus",
    },
    Book {
        id: 92,
        title: "The Fall",
        author: "Albert Camus",
    },
    Book {
        id: 93,
        title: "Nausea",
        author: "Jean-Paul Sartre",
    },
    Book {
        id: 94,
        title: "No Exit",
        author: "Jean-Paul Sartre",
    },
    Book {
        id: 95,
        title: "The Unbearable Lightness of Being",
        author: "Milan Kundera",
    },
    Book {
        id: 96,
        title: "The Book of Laughter and Forgetting",
        author: "Milan Kundera",
    },
    Book {
        id: 97,
        title: "One Day in the Life of Ivan Denisovich",
        author: "Aleksandr Solzhenitsyn",
    },
    Book {
        id: 98,
        title: "The Gulag Archipelago",
        author: "Aleksandr Solzhenitsyn",
    },
    Book {
        id: 99,
        title: "Doctor Zhivago",
        author: "Boris Pasternak",
    },
    Book {
        id: 100,
        title: "Lolita",
        author: "Vladimir Nabokov",
    },
];
