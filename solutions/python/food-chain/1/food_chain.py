def recite(start_verse, end_verse):
    animals = [
        ("fly", ""),
        ("spider", "It wriggled and jiggled and tickled inside her."),
        ("bird", "How absurd to swallow a bird!"),
        ("cat", "Imagine that, to swallow a cat!"),
        ("dog", "What a hog, to swallow a dog!"),
        ("goat", "Just opened her throat and swallowed a goat!"),
        ("cow", "I don't know how she swallowed a cow!"),
        ("horse", "")
    ]

    result = []
    for i in range(start_verse - 1, end_verse):
        animal, remark = animals[i]
        verse = []
        verse.append(f"I know an old lady who swallowed a {animal}.")
        if animal == "horse":
            verse.append("She's dead, of course!")
            result.extend(verse)
            if i != end_verse - 1:
                result.append("")
            continue
        if remark:
            verse.append(remark)
        for j in range(i, 0, -1):
            curr_animal = animals[j][0]
            prev_animal = animals[j - 1][0]
            if curr_animal == "bird":
                verse.append(
                    "She swallowed the bird to catch the spider that wriggled and jiggled and tickled inside her."
                )
            else:
                verse.append(f"She swallowed the {curr_animal} to catch the {prev_animal}.")
        verse.append("I don't know why she swallowed the fly. Perhaps she'll die.")
        result.extend(verse)
        if i != end_verse - 1:
            result.append("")

    return result