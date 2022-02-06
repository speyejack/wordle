
import string


def load_list(filename):
    with open(filename, "r") as fh:
        return [line.strip() for line in fh.readlines()]

def count_letters(word_list):
    max_size = max([len(word) for word in word_list])
    pos_count = [{letter:0 for letter in string.ascii_lowercase} for i in range(max_size)]
    tot_count = {letter:1 for letter in string.ascii_lowercase}

    for word in word_list:
        for i,letter in enumerate(word):
            tot_count[letter] += 1
            pos_count[i][letter] += 1

    tot_num = len(word_list)
    final_count = [{letter: 1*tot_count[letter]/(tot_num*max_size) + 3*(pos[letter]/tot_num) for letter in pos} for pos in pos_count]

    return final_count

def score_word(word, letter_count):
    letter_score = [(letter, letter_count[i][letter]) for i,letter in enumerate(word)]
    score_dict = {letter: 0 for letter in word}
    for letter,score in letter_score:
        score_dict[letter] = max(score_dict[letter],score)

    return sum(score_dict.values())

def score_wordlist(word_list, letter_count):
    scored_words = [(score_word(word,letter_count), word) for word in word_list]
    return sorted(scored_words)

def filter_word(word, pos=[None]*5, nopos=[[]]*5, inc=[], exc=[], size=(5,5)):
    if not (size[0] <= len(word) <= size[1]):
        return False

    if not all([(letter in word) for letter in inc]):
        return False

    if not all([(letter not in word) for letter in exc]):
        return False

    if not all([(letter is pos_letter) for letter,pos_letter in zip(word,pos) if pos_letter]):
        return False

    if not all([(letter not in no_pos_list) for letter,no_pos_list in zip(word,nopos)]):
        return False

    return True



def filter_list(word_list, *args, **kwargs):
    return [word for word in word_list if filter_word(word, *args, **kwargs)]

def tfil(word_list, *args, **kwargs):
    lst = filter_list(word_list, *args, **kwargs)
    plist(lst)

def plist(word_list):
    print("\n".join(str(word) for word in word_list[-10:]))

def score_words(word_list, letter_count):
    return [(score_word(word, letter_count), word) for word in word_list]


if __name__ == "__main__":
    root_word_list = load_list("/home/jack/Documents/jordle/words/answers.txt")
    # root_word_list = load_list("/home/jack/Downloads/words.txt")
    word_list = filter_list(root_word_list)
    # print(word_list)


    exc = ""
    inc = ""
    pos=[None]*5
    nopos=[[]]*5

    while len(word_list) > 1:
        letters = count_letters(word_list)
        scored = score_wordlist(word_list, letters)

        print(f"{len(scored)} options left.\n")
        plist(scored)

        breakpoint()
        word_list = filter_list(word_list, exc=exc, inc=inc, pos=pos, nopos=nopos)

    print(word_list)
    print("Solved!")
