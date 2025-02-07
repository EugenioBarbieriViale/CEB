import numpy as np

str_eq = str(input("Enter your equation: ")).replace(" ", "") + "+"

def coefficients(str_eq):
    ans = []
    chunk = np.array([])

    for i in range(len(str_eq)):
        if i+2 < len(str_eq) and str_eq[i].isupper() and str_eq[i+1].islower():
            if str_eq[i+2].isdigit():
                chunk = np.append(chunk, int(str_eq[i+2]))
            else:
                chunk = np.append(chunk, 1)

        elif i+1 < len(str_eq) and str_eq[i].isupper():
            if str_eq[i+1].isdigit():
                chunk = np.append(chunk, int(str_eq[i+1]))
            else:
                chunk = np.append(chunk, 1)

        if str_eq[i] == "+" or str_eq[i] == "=":
            ans.append(chunk)
            chunk = np.array([])

        if str_eq[i] == "=":
            ans.append(np.array([-1]))

    return np.array(ans, dtype=object)

def elements(str_eq):
    ans = []
    chunk = np.array([])

    for i in range(len(str_eq)):
        if str_eq[i].isupper() and not str_eq[i+1].islower():
            chunk = np.append(chunk, str(str_eq[i]))
        elif str_eq[i].isupper() and str_eq[i+1].islower():
            chunk = np.append(chunk, str(str_eq[i])+str(str_eq[i+1]))

        if str_eq[i] == "+" or str_eq[i] == "=":
            ans.append(chunk)
            chunk = np.array([])

        if str_eq[i] == "=":
            ans.append(np.array([-1]))

    return np.array(ans, dtype=object)

coef = coefficients(str_eq)
elems = elements(str_eq)

def verify(encoded, tokens):

    if encoded.shape != tokens.shape:
        print("Not same shape: ", encoded.shape, tokens.shape)
        return -1

    ans = []
    for i in range(encoded.shape[0]):
        for t, e in zip(tokens[i], encoded[i]):
            print(t, e)

verify(coef, elems)
# print(verify(coef, elems))
