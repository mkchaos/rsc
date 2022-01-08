int a = 4;
int b = 3 + 5;

int foo(int);

int foo(int a) {}

int main()
{
    int a = foo(1);
    int b = a;
}