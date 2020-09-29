

A view model represents the data that you want to display on your 
view/page, whether it be used for static text or for input values (like 
textboxes and dropdown lists) that can be added to the database (or 
edited). It is something different than your domain model. It is a model 
for the view.


A ViewModel is a "Model of a View".

Your ViewModel should expose the actual entity object(s) rather than it's 
individual properties. E.g. if you have a Customer entity with Forename, 
Surname and Age properties you would have a Customer SelectedCustomer 
property or List\<Customer\> AllCustomers property rather than Forename, 
Surname, Age etc.

You set your View's DataContext to an instance of the required ViewModel 
and use binding to view/update the various properties of the ViewModel, 
in this case Customer.Forename, Customer.Surname etc.

The ViewModel would implement INotifyPropertyChanged interface to raise 
messages when any of it's properties change, to notify the View of any 
changes and update the View.

