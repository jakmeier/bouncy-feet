const adjectives = [
    'Animated', 'Blessed', 'Blissful', 'Bouncy', 'Bubbly', 'Charming', 'Cheerful', 'Clever', 'Creative', 'Curious',
    'Daring', 'Dazzling', 'Delightful', 'Dynamic', 'Elated', 'Enchanting', 'Energetic', 'Exuberant', 'Fanciful', 'Funky',
    'Giddy', 'Gleaming', 'Gleeful', 'Glorious', 'Glowing', 'Graceful', 'Lucky', 'Happy', 'Jovial', 'Jubilant', 'Jumpy',
    'Lighthearted', 'Lively', 'Magnetic', 'Merry', 'Perky', 'Playful', 'Quaint', 'Quirky', 'Radiant', 'Serene',
    'Silly', 'Snappy', 'Snazzy', 'Sparkling', 'Spirited', 'Spontaneous', 'Sprightly', 'Spunky', 'Tenacious', 'Vibrant',
    'Vivacious', 'Whimsical', 'Whirlwind', 'Witty', 'Wondrous', 'Zany', 'Zesty', 'Zippy'
];

const subjects = [
    'Armadillo', 'Beetle', 'Bird', 'Boar', 'Bug', 'Bunny', 'Cat', 'Cheetah', 'Cow', 'Daemon',
    'Dog', 'Dolphin', 'Dragon', 'Duck', 'Dwarf', 'Elephant', 'Elf', 'Fox', 'Giraffe',
    'Goat', 'Gorilla', 'Grasshopper', 'Hippopotamus', 'Jaguar', 'Kangaroo', 'Kitten', 'Koala', 'Leopard', 'Lion',
    'Lynx', 'Orc', 'Otter', 'Owl', 'Panda', 'Parrot', 'Pegasus', 'Penguin', 'Pig',
    'Puppy', 'Rabbit', 'Raccoon', 'Shark', 'Sheep', 'Snow Leopard', 'Tiger', 'Tortoise', 'Troll', 'Turtle',
    'Unicorn', 'Vampire', 'Witch', 'Wizard', 'Wolf', 'Zebra', 'Zombie',
];

export function generateRandomUsername() {
    const i = Math.floor(Math.random() * adjectives.length);
    const j = Math.floor(Math.random() * subjects.length);

    return `${adjectives[i]} ${subjects[j]}`;
}