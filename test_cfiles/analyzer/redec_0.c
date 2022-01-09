int main()
{
    int a = 1;
    {
        int a = 1;
    }
    int a = 1; // redeclare here
}