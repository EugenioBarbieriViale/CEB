import numpy as np

str_eq = str(input("Enter your equation: ")).replace(" ", "") + "+"

def coefficients(str_eq):
    ans = []
    chunk = []

    for i in range(len(str_eq)):
        if i+2 < len(str_eq) and str_eq[i].isupper() and str_eq[i+1].islower():
            if str_eq[i+2].isdigit():
                chunk.append(int(str_eq[i+2]))
            else:
                chunk.append(1)

        elif i+1 < len(str_eq) and str_eq[i].isupper():
            if str_eq[i+1].isdigit():
                chunk.append(int(str_eq[i+1]))
            else:
                chunk.append(1)

        if str_eq[i] == "+" or str_eq[i] == "=":
            ans.append(chunk)
            chunk = []

        if str_eq[i] == "=":
            ans.append([-1])

    return ans

def elements(str_eq):
    ans = []
    chunk = []

    for i in range(len(str_eq)):
        if str_eq[i].isupper() and not str_eq[i+1].islower():
            chunk.append(str(str_eq[i]))
        elif str_eq[i].isupper() and str_eq[i+1].islower():
            chunk.append(str(str_eq[i])+str(str_eq[i+1]))

        if str_eq[i] == "+" or str_eq[i] == "=":
            ans.append(chunk)
            chunk = []

        if str_eq[i] == "=":
            ans.append([-1])

    return ans

coeff = coefficients(str_eq)
elems = elements(str_eq)
print(coeff)
print(elems)

def set_eq(elems, coeff):
    if len(elems) != len(coeff):
        print("Not same shape: ", len(elems), len(coeff))
        return -1

    ans = []
    eq_index = elems.index([-1])

    for i in range(eq_index):
        for j in range(len(elems) - eq_index):
            for k in range(len(elems[i])):
                for l in range(len(elems[j])):
                    if elems[i][k] == elems[j][l]:
                        ans.append([coeff[i][k], elems[i][k], coeff[j][l], elems[j][l]])

    return ans

e = set_eq(elems, coeff)
print("\n")
print(e)
