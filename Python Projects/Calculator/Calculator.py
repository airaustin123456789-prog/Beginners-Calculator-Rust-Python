
def main():

    print("")

    user_operator = input("Choose the operator - M, D, A, S, Q: ").lower()

    if user_operator == "m":

        import Multiplication

        main()

    
    elif user_operator == "d":

        import Division

        main()


    elif user_operator == "a":

        import Addition

        main()
    
    elif user_operator == "s":

        import Subtraction

        main()

    
    elif user_operator == "q":

        print("")

        print("Thanks for using my calculator!")

        exit()


    else:

        print("Please enter a valid operator!")

        main()


main()