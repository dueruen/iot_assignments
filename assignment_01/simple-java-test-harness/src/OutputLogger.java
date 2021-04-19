import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;
import org.json.*;

class OutputLogger extends Logger {
    BufferedReader input;
    
    public OutputLogger (String filename, String separator, Process p) throws IOException {
        super(filename, separator);
        input = new BufferedReader(new InputStreamReader(p.getInputStream()));
        header(new String[] {"line"});
    }
    
    public void run () {
        try {
            String line;
            while ((line = input.readLine()) != null) {
                log(line);
            }
        } catch (IOException e) {
            System.out.println("OutputLogger("+filename+") caught exception: "+e);
        }
        try {
            finalize();
        } catch (IOException e) {
            System.out.println("OutputLogger("+filename+") caught finalize exception: "+e);
        }
    }

    @Override
    protected void log (String line) throws IOException {
       JSONObject jo = new JSONObject(new String(line.split(" ")[1]));
        double time  = jo.getDouble("time");
        double value = jo.getDouble("value");
        long current = jo.getLong("current");

        long diff = System.currentTimeMillis() - current;

        writer.write(diff+sep+line+System.lineSeparator()); //+sep+line+System.lineSeparator()
    }
}
