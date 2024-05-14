def stray(arr):
    if arr.count(arr[0]) >1:
        return int("".join(arr).replace(arr[0], ""))
    return arr[0]

stray_lamb = lambda arr: int("".join(arr).replace(arr[0], "")) if arr.count(arr[0]) >1 else arr[0]

print(stray([1,2,3]) == stray_lamb([1,2,3]) )










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

