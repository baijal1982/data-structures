/*
 * This class pprints all permuations of a given string 
 */
public class Permutation  {

    
    public static void permutate(String soFar, String remaining)  {
        if(remaining=="") {
            System.out.println(soFar);
        }
        for ( int count =0 ; count< remaining.length() ;count++ )   {
           String next = soFar + remaining.charAt(count);
           String left = remaining.substring(0, count)+ remaining.substring(count+1) ;
           permutate(next, left);

        }
    }



    public static void main(String arg[])   {
        permutate("", "listen");
    }
}