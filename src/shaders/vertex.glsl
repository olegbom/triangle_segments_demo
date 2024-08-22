#version 100
attribute vec2 pos;
attribute float index;

uniform vec2 offset;
uniform float aspect;
uniform ivec4 bitfield;

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
    else if ( bit < 4 )
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
    else if( bit < 8 ) 
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
       
    return false;
}


void main() {
    vec2 p = pos + offset;
    p.y *= aspect;

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

    if( i < 8 )
    {
        is_draw = get_bit(bitfield.x, int(i)) ;
    }
    else if (i < 16)
    {
        is_draw = get_bit(bitfield.y, int(i - 8)) ;
    }
    else if( i < 24 )
    {
        is_draw = get_bit(bitfield.z, int(i - 16)) ;
    }
    else
    {
        is_draw = get_bit(bitfield.w, int(i - 24)) ;
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
