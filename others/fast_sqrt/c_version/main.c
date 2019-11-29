#include <stdio.h>
int main()
{
  float InvSqrt(float x);
  float InvSqrt1(float x);
  float Q_rsqrt( float number );
  float InvSqrt3 (float x);

  float a = 100.0;
  float res =  InvSqrt(a);
  
  printf("%f\n",res);
  return 0;
}

float InvSqrt(float x)
{
  float xhalf = 0.5f*x;
  printf("xhalf: %f\n", xhalf);
  int i = *(int*)&x;
  printf("i1 int: %i\n", i);
  printf("i1 float: %f\n",(float) i);
  printf("i1 float: %f\n",*(float*) &i);
  printf("i1 0x : %#010x\n", i);

  printf("i>>1 0x : %#010x\n", i>>1);
  printf("i>>1 int: %i\n", i>>1);

  i = 0x5f375a86 - (i>>1);

  printf("i2 int: %i\n", i);
  printf("i2 0x : %#010x\n", i);

  printf("0x5f375a86: 0x: %#010x\n", 0x5f375a86);
  printf("0x5f375a86: 10:%i\n", 0x5f375a86);  
  x = *(float*)&i;
  printf("x int: %f\n", x);



  //x = x*(1.5f-xhalf*x*x);
  //x = x*(1.5f-xhalf*x*x);

  return 1/x;
}
/*
float InvSqrt1(float x)
{
  float xhalf = 0.5f*x;
  int i = *(int*)&amp;x;
  i = 0x5f3759df - (i&gt;&gt;1);
  x = x*(1.5f -xhalf*x*x);
  return x;
}
*/
float Q_rsqrt( float number )
{
  long i;
  float x2, y;
  const float threehalfs = 1.5F;

  x2 = number * 0.5F;
  y  = number;
  i  = * ( long * ) &y;
  i  = 0x5f3759df - ( i >> 1 );
  y  = * ( float * ) &i;
  y  = y * ( threehalfs - ( x2 * y * y ) );


  return y;
}


float InvSqrt3 (float x){
    float xhalf = 0.5f*x;
    int i = *(int*)&x;
    i = 0x5f3759df - (i>>1);
    x = *(float*)&i;
    x = x*(1.5f - xhalf*x*x);
    return x;
}
