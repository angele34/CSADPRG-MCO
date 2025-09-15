package main

import "fmt"

/*
TODO: Restructure the main code into separate functions for 
input, calculation, numerical to string conversion for output
*/
// func getInput() {

// }

// Convert to string

// func calculate() {

// }

func main() {
	var loanAmount float64 // Total loan amount in PHP
	var annualInterestRate float64 // Percentage rate
	var loanTermYears int8 // Loan duration in years

	// Input
	fmt.Print("Loan Amount: ")
	fmt.Scan(&loanAmount)

	fmt.Print("Annual Interest Rate: ")
	fmt.Scan(&annualInterestRate)

	fmt.Print("Loan Term: ")
	fmt.Scan(&loanTermYears);

	// Get the monthly interest rate
	monthlyInterestRate := (annualInterestRate / 100) / 12

	// Get the loan term in months
	loanTermMonths := loanTermYears * 12

	// Get the total interest
	totalInterest := loanAmount * monthlyInterestRate * float64(loanTermMonths)

	// Get the monthly repayment
	monthlyRepayment := (loanAmount + totalInterest) / float64(loanTermMonths)

	// Print output
	fmt.Println()
	fmt.Println("Loan Amount: PHP", loanAmount)
	fmt.Printf("Annual Interest Rate: %.f%%\n", annualInterestRate)
	fmt.Println("Loan Term:", loanTermMonths, "months")
	fmt.Printf("Monthly Repayment: PHP %.2f\n", monthlyRepayment)
	fmt.Println("Total Interest: PHP", totalInterest)




}