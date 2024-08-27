#version 100
attribute vec2 pos;
attribute float index;
attribute vec2 offset;

uniform vec2 scale;
uniform ivec2 bitfield;

bool is_odd(int n)
{
    return (n - (n / 2) * 2) > 0;
}

bool get_bit(int field, int bit)
{
    if( bit < 0 ) 
    {
        return false;
    }
    if( bit < 8 )
    {
        if ( bit < 4 )
        {
            if( bit < 2 )
            {
                if( bit == 0 )
                {
                    return is_odd(field);
                }
                else // == 1
                {
                    return is_odd(field / 2);
                }
            }
            else
            {
                if( bit == 2 )
                {
                    return is_odd(field / 4);
                }
                else // == 3
                {
                    return is_odd(field / 8);
                }   
            }
        }
        else  
        {
            if( bit < 6 )
            {
                if( bit == 4 )
                {
                    return is_odd(field / 16);
                } 
                else // == 5
                {
                    return is_odd(field / 32);
                }
            }
            else 
            {
                if( bit == 6 )
                {
                    return is_odd(field / 64);
                }
                else // == 7
                {
                    return is_odd(field / 128);
                }
            }
        }
    }
    else if( bit < 16 )
    {
        if ( bit < 12 )
        {
            if( bit < 10 )
            {
                if( bit == 8 )
                {
                    return is_odd(field / 256);
                }
                else // == 9
                {
                    return is_odd(field / 512);
                }
            }
            else
            {
                if( bit == 10 )
                {
                    return is_odd(field / 1024);
                }
                else // == 11
                {
                    return is_odd(field / 2048);
                }   
            }
        }
        else  
        {
            if( bit < 14 )
            {
                if( bit == 12 )
                {
                    return is_odd(field / 4096);
                } 
                else // == 13
                {
                    return is_odd(field / 8192);
                }
            }
            else 
            {
                if( bit == 14 )
                {
                    return is_odd(field / 16384);
                }
                else // == 15
                {
                    return is_odd(field / 32768);
                }
            }
        }   
    }
    return false;
}


void main() {
    vec2 p = pos + offset;
    p *= scale;

    int i = int(index)/3;
    if( i < 36 )
    {
        i = i/2;
    }
    else
    {
        i = 18 + (i - 36);
    }

    bool is_draw = false;

    if( i < 16 )
    {
        is_draw = get_bit(bitfield.x, int(i)) ;
    }
    else 
    {
        is_draw = get_bit(bitfield.y, int(i - 16)) ;
    }
   

    if( is_draw )
    {
        gl_Position = vec4(p, 0., 1.);
    }
    else 
    {
        gl_Position = vec4(0., 0., 0., 1.);
    }
}
