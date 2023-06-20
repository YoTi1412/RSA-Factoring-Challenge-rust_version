# RSA Factoring Challenge :cat:
This repo is just an attempt to read in and factorize a file of numbers.

## Tasks
* "Task 0" : 
Factorize as many numbers as possible into a product of two smaller numbers.

* Usage: factors <file>
    + where <file> is a file containing natural numbers to factor.
    + One number per line
    + you can assume that all lines will be valid natural numbers greater than 1
    + You can assume that there will be no empy line, and no space before and after the valid number
    + The file will always end with a new line
* Output format: n=p*q
    + one factorization per line
    + p and q don’t have to be prime numbers
* You can work on the numbers of the file in the order of your choice
* Your program should run without any dependency: You will not be able to install anything on the machine we will run your program on
* Time limit: Your program will be killed after 5 seconds if it hasn’t finish
* Push all your scripts, source code, etc… to your repository
* we will only run your executable factors

* "Task 1" :
RSA Laboratories states that: for each RSA number n, there exist prime numbers p and q such that

n = p × q. The problem is to find these two primes, given only n.

This task is the same as task 0, except:

* p and q are always prime numbers
* There is only one number in the files

How far can you go in less than 5 seconds?
* Read: [RSA Factoring Challenge](https://en.wikipedia.org/wiki/RSA_Factoring_Challenge)

## Description of what each file shows:
* factors - the program to factorize 
* tests/test00 - the file with the numbers

* rsa - same as "factors" except : 
  + p and q are always prime numbers
  + There is only one number in the files
* tests/rsa-* - files with a number, and other with a list of numbers 

### Environment
* Language: rust programming language :heart:
* OS: macOS Monterey 12.6.6 | Ubuntu 20.04 LTS

### Usage
```
./factors tests/test00

./rsa tests/rsa-"any file you want from rsa"
```

## Author :octocat:
[GitHub](https://github.com/YoTi1412) | [Twitter](https://twitter.com/YoTi1412) 
