def sort_array(source_array):
    if len(source_array) < 2: 
        return source_array
    else:
        array = sorted(source_array)
        odd_list = []
        
        ##remove all the odd numbers and store their value and index in odd dict
        
        for number in source_array:
            if number % 2 != 0:
                print(number)
                odd_list.append(number)
                array.remove(number)
        print(odd_list)
        odd_list.sort()
        
        for i,number in enumerate(source_array):
            if number not in odd_list:
                odd_list.insert(i,number)
        print(odd_list)
    return odd_list
#print(sort_array([5, 3, 1, 8, 0]))




def stray(arr):
    if arr.count(arr[0]) >1:
        return int("".join(arr).replace(arr[0], ""))
    return arr[0]

stray_lamb = lambda arr: int("".join(arr).replace(arr[0], "")) if arr.count(arr[0]) >1 else arr[0]

#print(stray([1,2,3]) == stray_lamb([1,2,3]) )


# def count(s):
#     acc = {}
#     for char in s:
#         print(f"Char is {char} and acc is {acc}")
#         if char not in acc.keys():
#             acc.update({char: 1})
#         else:
#             acc[char] += 1
#     return acc
def count(s):
    return { char: s.count(char) for char in s } #dict()


print(count("aba"))






def increment_string(string):
    acc = [ ]
    for char in string[::-1]:
        if char.isdigit():
            acc.append(char)
        else:
            break
    print(acc)
    if acc == []:
        
        return string + str(1)
    else:
        rev_acc = acc[::-1]
        print(rev_acc)
        join_rev_acc = "".join(rev_acc)
        rev_acc_length = len(rev_acc)
        print(join_rev_acc)

        acc_int = int(join_rev_acc) + 1
        missing_0 = rev_acc_length - len(str(acc_int))
        print(acc_int)
        replaced_rev_acc = string[:-rev_acc_length] #string.replace(str(join_rev_acc), "")
        print(replaced_rev_acc)
        return replaced_rev_acc + str(missing_0*"0")+ str((acc_int))

import requests

auth = ("user_id", "jenkins_token")